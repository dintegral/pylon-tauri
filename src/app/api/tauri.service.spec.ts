import { TauriService } from './tauri.service';

describe('TauriService', () => {
  let service: TauriService;

  beforeEach(() => {
    service = new TauriService();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
