import { Device } from '@/types/api';
import {create} from 'zustand';

export interface DeviceData {
    id: string,
    device: Device
}

interface DeviceStore {
  devices: DeviceData[];
  setDevices: (devices: DeviceData[]) => void;
}

export const useDeviceStore = create<DeviceStore>((set) => ({
    devices: [],
    setDevices: (devices) => set({devices})
}))