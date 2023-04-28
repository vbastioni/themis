import { ConfigService } from "@nestjs/config";
import { ElasticConfig } from "./interfaces/elastic-config.interface";

export function getElasticConfig(configService: ConfigService): ElasticConfig {
    return {
        host: configService.get<string>("ES_HOST", "localhost"),
        port: configService.get<number>("ES_PORT", 9200),
        user: configService.get<string>("ES_USER", "elastic"),
        pass: configService.getOrThrow<string>("ES_PASS"),
        cert: configService.getOrThrow<string>("ES_CERT"),
    };
}

//Camille Fantini
//shirley almosni chiche
// System Design Primer 1 & 2

/*

marguerite@ignition-program.com
Camille Fantini
shirley almosni chiche
https://www.workingnomads.com/jobs
https://cord.co/

*/