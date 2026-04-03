/** IndexedDB persistence for sampler pad samples. */

interface StoredSample {
    id: string;
    padIndex: number;
    left: ArrayBuffer;
    right: ArrayBuffer;
    sampleRate: number;
    length: number;
    name: string;
}

let db: IDBDatabase | null = null;

async function openDB(): Promise<IDBDatabase> {
    if (db) return db;
    return new Promise((resolve, reject) => {
        const request = indexedDB.open('orbit-sampler', 1);
        request.onupgradeneeded = (e) => {
            const database = (e.target as IDBOpenDBRequest).result;
            if (!database.objectStoreNames.contains('samples')) {
                database.createObjectStore('samples', { keyPath: 'id' });
            }
        };
        request.onsuccess = (e) => {
            db = (e.target as IDBOpenDBRequest).result;
            resolve(db);
        };
        request.onerror = () => reject(request.error);
    });
}

export async function saveSample(
    padIndex: number,
    left: Float32Array,
    right: Float32Array,
    sampleRate: number,
    name: string,
): Promise<void> {
    const database = await openDB();
    const tx = database.transaction('samples', 'readwrite');
    const store = tx.objectStore('samples');
    store.put({
        id: `pad-${padIndex}`,
        padIndex,
        left: left.buffer.slice(0),
        right: right.buffer.slice(0),
        sampleRate,
        length: left.length,
        name,
    } as StoredSample);
    return new Promise((resolve, reject) => {
        tx.oncomplete = () => resolve();
        tx.onerror = () => reject(tx.error);
    });
}

export async function loadAllSamples(): Promise<Map<number, { left: Float32Array; right: Float32Array; sampleRate: number; name: string }>> {
    const database = await openDB();
    const tx = database.transaction('samples', 'readonly');
    const store = tx.objectStore('samples');
    const request = store.getAll();
    return new Promise((resolve, reject) => {
        request.onsuccess = () => {
            const result = new Map();
            for (const record of (request.result as StoredSample[])) {
                result.set(record.padIndex, {
                    left: new Float32Array(record.left),
                    right: new Float32Array(record.right),
                    sampleRate: record.sampleRate,
                    name: record.name,
                });
            }
            resolve(result);
        };
        request.onerror = () => reject(request.error);
    });
}

export async function deleteSample(padIndex: number): Promise<void> {
    const database = await openDB();
    const tx = database.transaction('samples', 'readwrite');
    tx.objectStore('samples').delete(`pad-${padIndex}`);
    return new Promise((resolve, reject) => {
        tx.oncomplete = () => resolve();
        tx.onerror = () => reject(tx.error);
    });
}
