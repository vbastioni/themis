import { Controller, Get, Res } from '@nestjs/common';
import { SearchService } from './search.service';
import { Response } from 'express';

@Controller('search')
export class SearchController {
    constructor(private searchService: SearchService) { }

    @Get()
    get() {
        return this.searchService.version;
    }

    @Get('test')
    async test(@Res() res: Response) {
        const results = await this.searchService.test();
        return res.json(results).send();
    }
}
