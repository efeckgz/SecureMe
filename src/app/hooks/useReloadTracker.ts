// utils/reloadTracker.ts
import { useEffect } from "react";

interface ReloadDetails {
  timestamp: number;
  url: string;
  source: "beforeunload" | "unload" | "pagehide" | "visibilitychange";
  performance?: {
    navigationTiming: Partial<PerformanceNavigationTiming>;
    navigation: {
      type: number;
      typeString: string;
    };
  };
}

export const useReloadTracker = (
  onReload?: (details: ReloadDetails) => void
) => {
  useEffect(() => {
    let isReloading = false;

    const getNavigationTypeString = (type: number): string => {
      switch (type) {
        case 0:
          return "navigate";
        case 1:
          return "reload";
        case 2:
          return "back_forward";
        case 255:
          return "prerender";
        default:
          return "unknown";
      }
    };

    const createReloadDetails = (
      source: ReloadDetails["source"]
    ): ReloadDetails => {
      const details: ReloadDetails = {
        timestamp: Date.now(),
        url: window.location.href,
        source,
      };

      // Add performance data if available
      if (performance && performance.getEntriesByType) {
        const navigationTiming = performance.getEntriesByType(
          "navigation"
        )[0] as PerformanceNavigationTiming;
        const navigation = performance.navigation;

        if (navigationTiming || navigation) {
          details.performance = {
            navigationTiming: navigationTiming
              ? {
                  type: navigationTiming.type,
                  redirectCount: navigationTiming.redirectCount,
                  // activationStart: navigationTiming.activationStart,
                }
              : {},
            navigation: {
              type: navigation?.type ?? -1,
              typeString: getNavigationTypeString(navigation?.type ?? -1),
            },
          };
        }
      }

      return details;
    };

    const handleBeforeUnload = (e: BeforeUnloadEvent) => {
      isReloading = true;
      const details = createReloadDetails("beforeunload");
      onReload?.(details);
      console.log("Page reload detected (beforeunload):", details);
    };

    const handleUnload = (e: Event) => {
      if (isReloading) {
        const details = createReloadDetails("unload");
        onReload?.(details);
        console.log("Page reload detected (unload):", details);
      }
    };

    const handlePageHide = (e: PageTransitionEvent) => {
      if (isReloading) {
        const details = createReloadDetails("pagehide");
        onReload?.(details);
        console.log("Page reload detected (pagehide):", details);
      }
    };

    const handleVisibilityChange = () => {
      if (document.visibilityState === "hidden" && isReloading) {
        const details = createReloadDetails("visibilitychange");
        onReload?.(details);
        console.log("Page reload detected (visibilitychange):", details);
      }
    };

    // Attach event listeners
    window.addEventListener("beforeunload", handleBeforeUnload);
    window.addEventListener("unload", handleUnload);
    window.addEventListener("pagehide", handlePageHide);
    document.addEventListener("visibilitychange", handleVisibilityChange);

    // Cleanup
    return () => {
      window.removeEventListener("beforeunload", handleBeforeUnload);
      window.removeEventListener("unload", handleUnload);
      window.removeEventListener("pagehide", handlePageHide);
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [onReload]);
};
