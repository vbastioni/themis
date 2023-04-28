import { Injectable } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { ElasticsearchService } from '@nestjs/elasticsearch';

@Injectable()
export class SearchService {
    constructor(
        private readonly elasticsearchService: ElasticsearchService,
    ) {
    }

    private get service() {
        return this.elasticsearchService;
    }

    get version() {
        return this.service.info();
    }

    async test() {
        const indexName = 'game-of-thrones';
        const uid = 'fc52d615-e139-43c6-a53c-a6c98f877cdd';

        await this.service.indices.delete({
            index: indexName,
        });
        await this.service.index({
            index: indexName,
            id: uid,
            document: {
                uid,
                character: 'Daenerys Targaryen',
                quote: 'I am the blood of the dragon.'
            }
        });

        await this.service.indices.refresh({ index: indexName });

        return await this.service.search({
            index: indexName,
        });
    }
}
