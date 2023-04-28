import { createRoot } from 'react-dom/client';
import { Routes } from '@generouted/react-router';
import { QueryClient, QueryClientProvider, } from 'react-query';

const container = document.getElementById('app')!;

const client = new QueryClient();

createRoot(container).render(
  <QueryClientProvider client={client}>
    <Routes />
  </QueryClientProvider>
);