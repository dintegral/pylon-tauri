import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, Component } from '@angular/core';
import { ApiService } from '../api';
import { DappListing } from '../model';
import {
  RowComponent,
  ColumnComponent,
  ContainerComponent,
  DappListingCardComponent,
} from '../ui';

const DAPP_LISTINGS: DappListing[] = [
  {
    name: 'NNS',
    canisterId: 'qoctq-giaaa-aaaaa-aaaea-cai',
    description: 'Asset Management and Voting for the Network Nervous System',
  },
  {
    name: 'Internet Identity',
    canisterId: 'rdmx6-jaaaa-aaaaa-aaadq-cai',
    description: 'Anonymous blockchain authentication framework',
  },
];

@Component({
  selector: 'app-dashboard',
  imports: [
    CommonModule,
    ContainerComponent,
    RowComponent,
    ColumnComponent,
    DappListingCardComponent,
  ],
  standalone: true,
  templateUrl: './dashboard.component.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class DashboardComponent {
  public dappListings = DAPP_LISTINGS;

  constructor(private readonly apiService: ApiService) {}

  public async onOpenDapp(dappListing: DappListing): Promise<void> {
    this.apiService.openDapp(dappListing.canisterId);
  }
}
