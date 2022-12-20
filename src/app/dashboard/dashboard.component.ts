import { ChangeDetectionStrategy, Component } from '@angular/core';
import { ContainerComponent } from '../ui/container';

@Component({
  standalone: true,
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [ContainerComponent],
})
export class DashboardComponent {}
