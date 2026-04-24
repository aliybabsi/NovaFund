import { Module } from '@nestjs/common';
import { HttpModule } from '@nestjs/axios';
import { RpcFallbackService } from './rpc-fallback.service';
import { RpcFallbackController } from './rpc-fallback.controller';
import { PathfinderService } from './pathfinder.service';
import { FederationService } from './federation.service';
import { FederationController } from './federation.controller';
import { AssetDiscoveryService } from './asset-discovery.service';
import { AssetDiscoveryController } from './asset-discovery.controller';
import { PrismaService } from '../prisma.service';
import { ScheduleModule } from '@nestjs/schedule';

@Module({
  imports: [HttpModule, ScheduleModule.forRoot()],
  providers: [
    RpcFallbackService,
    PathfinderService,
    FederationService,
    AssetDiscoveryService,
    PrismaService
  ],
  controllers: [
    RpcFallbackController,
    FederationController,
    AssetDiscoveryController
  ],
  exports: [
    RpcFallbackService,
    PathfinderService,
    FederationService,
    AssetDiscoveryService
  ],
})
export class StellarModule {}
