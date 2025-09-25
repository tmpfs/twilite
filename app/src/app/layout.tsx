import type { Metadata } from "next";
import "./globals.css";
import { Suspense } from "react";
import { Header } from "@/components/Header";
import { ThemeProvider } from "@/components/theme-provider";
import { Toaster } from "@/components/ui/sonner";
import { ToastProvider } from "@/context/toast";

export const metadata: Metadata = {
  title: "Litewiki",
  description: "Lightweight wiki",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <ThemeProvider
        attribute="class"
        defaultTheme="system"
        enableSystem
        disableTransitionOnChange
      >
        <body className="flex flex-col">
          <Header />
          <main>
            <Suspense fallback={null}>
              <ToastProvider>{children}</ToastProvider>
            </Suspense>
          </main>
          <footer></footer>
          <Toaster position="top-center" className="font-sans" />
        </body>
      </ThemeProvider>
    </html>
  );
}
