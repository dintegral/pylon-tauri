import { ApiService } from './api.service';
import { TauriService } from './tauri.service';

describe('ApiService', () => {
  let service: ApiService;
  let tauriServiceMock: jasmine.SpyObj<TauriService>;

  beforeEach(() => {
    tauriServiceMock = jasmine.createSpyObj<TauriService>('tauriService', [
      'invoke',
    ]);

    service = new ApiService(tauriServiceMock);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
