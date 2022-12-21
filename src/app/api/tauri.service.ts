import { Injectable } from '@angular/core';
import { invoke, InvokeArgs } from '@tauri-apps/api/tauri';

@Injectable({
  providedIn: 'root',
})
export class TauriService {
  public async invoke<R = {}, P extends InvokeArgs = {}>(
    cmd: string,
    payload?: P,
  ): Promise<R> {
    return await invoke<R>(cmd, payload);
  }
}
