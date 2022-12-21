import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, Component } from '@angular/core';
import { ApiService } from '../api';
import {
  RowComponent,
  CardComponent,
  ColumnComponent,
  ContainerComponent,
} from '../ui';

@Component({
  selector: 'app-dashboard',
  imports: [
    CommonModule,
    ContainerComponent,
    CardComponent,
    RowComponent,
    ColumnComponent,
  ],
  standalone: true,
  templateUrl: './dashboard.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DashboardComponent {
  constructor(private readonly apiService: ApiService) {}

  public async onDappCardClicked(): Promise<void> {
    this.apiService.openDapp();
  }
}
