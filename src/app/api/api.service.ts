import { Injectable } from '@angular/core';
import { TauriService } from './tauri.service';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  constructor(private readonly tauriService: TauriService) {}

  public openDapp(): Promise<void> {
    return this.tauriService.invoke('open_dapp');
  }
}
