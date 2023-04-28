// vite.config.ts

import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import generouted from '@generouted/react-router/plugin';
import path from 'path';

export default defineConfig({
    plugins: [react(), generouted()],
    resolve: {
        alias: {
            "@components": path.resolve(__dirname, "./src/components"),
            "@models": path.resolve(__dirname, "./src/models"),
        },
    },
});