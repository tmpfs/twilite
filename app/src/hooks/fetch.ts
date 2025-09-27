import { useState, useEffect } from "react";

type Fetcher<T> = () => Promise<T>;

type FetchState<T> =
  | { status: "loading" }
  | { status: "error"; error: Error }
  | { status: "success"; data: T };

export function useFetchWithDelay<T>(
  fetcher: Fetcher<T>,
  deps: any[] = [],
): FetchState<T> {
  const [state, setState] = useState<FetchState<T>>({ status: "loading" });
  const minDuration = 300;
  useEffect(() => {
    let cancelled = false;
    async function run() {
      setState({ status: "loading" });
      const start = Date.now();
      try {
        const result = await fetcher();
        const elapsed = Date.now() - start;
        const delay = Math.max(0, minDuration - elapsed);

        setTimeout(() => {
          if (!cancelled) {
            setState({ status: "success", data: result });
          }
        }, delay);
      } catch (err) {
        const elapsed = Date.now() - start;
        const delay = Math.max(0, minDuration - elapsed);

        setTimeout(() => {
          if (!cancelled) {
            setState({ status: "error", error: err as Error });
          }
        }, delay);
      }
    }

    run();

    return () => {
      cancelled = true;
    };
  }, deps);

  return state;
}
