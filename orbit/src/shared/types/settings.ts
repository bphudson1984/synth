export type SettingsParamType = 'slider' | 'toggle' | 'select';

export interface SettingsParam {
    name: string;
    id: number;
    min: number;
    max: number;
    default: number;
    type: SettingsParamType;
    options?: { value: number; label: string }[];
}

export interface SettingsSection {
    label: string;
    params: SettingsParam[];
}

export type QuickSlot = SettingsParam | null;
export const NUM_QUICK_SLOTS = 8;
