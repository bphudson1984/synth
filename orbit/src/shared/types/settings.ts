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
