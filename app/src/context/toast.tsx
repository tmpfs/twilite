// context/toast-context.tsx
"use client";

import React, { createContext, useContext, useEffect } from "react";
import { useRouter } from "next/navigation";
import { toast } from "sonner";

// Define the storage key
const TOAST_STORAGE_KEY = "flashToastMessage";

// Define the shape of the flash message data
type FlashMessageData = {
  type: "success" | "error" | "info" | "warning" | "default";
  title: string;
  description?: string;
};

// Define the context value (the function to trigger the toast and navigation)
type ToastContextType = {
  flashToastAndNavigate: (data: FlashMessageData, path: string) => void;
};

const ToastContext = createContext<ToastContextType | undefined>(undefined);

// The component that manages the toast display
export function ToastProvider({ children }: { children: React.ReactNode }) {
  const router = useRouter();

  // 🎯 Function to save message to sessionStorage and navigate
  const flashToastAndNavigate = (data: FlashMessageData, path: string) => {
    // 1. Save the message to sessionStorage
    sessionStorage.setItem(TOAST_STORAGE_KEY, JSON.stringify(data));

    // 2. Navigate
    // This happens immediately, but the toast message is safely persisted
    router.push(path);
  };

  // 🎯 Effect to display the toast on page load
  useEffect(() => {
    // Check if a message exists in sessionStorage
    const storedMessage = sessionStorage.getItem(TOAST_STORAGE_KEY);

    if (storedMessage) {
      try {
        // 1. Parse the stored message
        const { type, title, description }: FlashMessageData =
          JSON.parse(storedMessage);

        // 2. Display the toast
        if (type === "success") {
          toast.success(title, { description });
        } else if (type === "error") {
          toast.error(title, { description });
        } else {
          toast(title, { description });
        }

        // 3. CRITICAL: Clear the message immediately after displaying
        // This ensures it only shows once and is not displayed on manual refresh.
        sessionStorage.removeItem(TOAST_STORAGE_KEY);
      } catch (error) {
        console.error("Failed to parse sessionStorage flash message:", error);
        toast.error("An unknown error occurred.");
        sessionStorage.removeItem(TOAST_STORAGE_KEY); // Clear bad data
      }
    }
  }, [router]); // router is included as a dependency, though usually optional here

  return (
    <ToastContext.Provider value={{ flashToastAndNavigate }}>
      {children}
    </ToastContext.Provider>
  );
}

// Custom hook for components to use (remains the same)
export const useFlashToast = () => {
  const context = useContext(ToastContext);
  if (context === undefined) {
    throw new Error("useFlashToast must be used within a ToastProvider");
  }
  return context;
};
