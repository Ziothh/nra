import { appRouter, Routes } from '@acme/lib';


appRouter
  .useRouter()
  .push('/products/[id]/[variant]', {
    params: {
      id: 'hi',
      variant: 'hiiii'
    }
  });