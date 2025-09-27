import type { Metadata } from "next";
import "./globals.css";
import { Suspense } from "react";
import { Header } from "@/components/Header";
import { ThemeProvider } from "@/components/theme-provider";
import { Toaster } from "@/components/ui/sonner";
import { ToastProvider } from "@/context/toast";
import { SearchProvider } from "@/context/search";
import { SearchMenu } from "@/components/SearchMenu";

export const metadata: Metadata = {
  title: "Twilite",
  description: "Lightweight wiki",
  manifest: "/site.webmanifest",
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
        <SearchProvider>
          <body
            className="flex flex-col text-base sm:text-lg md:text-xl lg:text-2xl"
            id="top"
          >
            <Header />
            <main className="flex-1">
              <Suspense fallback={null}>
                <ToastProvider>{children}</ToastProvider>
              </Suspense>
            </main>
            <footer></footer>
            <Toaster position="top-center" className="font-sans" />
            <SearchMenu />
          </body>
        </SearchProvider>
      </ThemeProvider>
    </html>
  );
}
