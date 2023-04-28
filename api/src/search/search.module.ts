import { DynamicModule, Module } from '@nestjs/common';
import { SearchController } from './search.controller';
import { SearchService } from './search.service';
import { ElasticsearchModule } from '@nestjs/elasticsearch';
import { readFileSync } from 'fs';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { getElasticConfig } from './helper';

@Module({})
export class SearchModule {
  static register(): DynamicModule {
    return {
      module: SearchModule,
      imports: [ElasticsearchModule.registerAsync({
        imports: [ConfigModule],
        async useFactory(configService: ConfigService) {
          const { host, port, user, pass, cert } = getElasticConfig(configService);
          const file = readFileSync(cert);
          const url = new URL(`https://${host}:${port}`);
          return {
            node: url.href,
            auth: { username: user, password: pass },
            tls: { ca: file },
          }
        },
        inject: [ConfigService],
      })],
      controllers: [SearchController],
      providers: [SearchService],
    }
  }
}
