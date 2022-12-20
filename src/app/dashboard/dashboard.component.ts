import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, Component } from '@angular/core';
import { CardComponent } from '../ui/card';
import { ColumnComponent } from '../ui/column';
import { ContainerComponent } from '../ui/container';
import { RowComponent } from '../ui/row';

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
export class DashboardComponent {}
