import type { Metadata } from "next";
import "./globals.css";

import { Header } from "@/components/Header";
import { ThemeProvider } from "@/components/theme-provider";

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
          {children}
          </main>
          <footer>
            Footer
          </footer>

        </body>
      </ThemeProvider>

    </html>
  );
}
