import { Metadata } from "next";
import { Rethink_Sans, JetBrains_Mono } from "next/font/google";
import { Slide, ToastContainer } from "react-toastify";
import { Theme } from "@radix-ui/themes";

import { BitcoinWalletProvider } from "@/contexts/BitcoinWalletProvider";
import SolanaWalletProvider from "@/contexts/SolanaWalletProvider";
import { ZplClientProvider } from "@/contexts/ZplClientProvider";

import GlobalModals from "../components/GlobalModals/GlobalModals";
import { Nav } from "../components/Nav/Nav";

import "react-toastify/dist/ReactToastify.css";
import "@radix-ui/themes/styles.css";
import "./globals.scss";
import "./design-system.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://z-fubao.netlify.app"),
  title: "Z-Fubao",
  description:
    "Z-Fubao: Earn, Pay, and Live with your Bitcoin - A DeFi protocol built on Solana for ZBTC collateralization and ZUSD staking",
  openGraph: {
    images: ["/zFuBao_logo.png"],
    title: "Z-Fubao",
  },
  twitter: {
    images: ["/zFuBao_logo.png"],
    card: "summary_large_image",
  },
};

const rethinkSans = Rethink_Sans({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-rethink-sans",
  weight: ["400", "500", "600", "700", "800"],
  style: ["normal", "italic"],
  adjustFontFallback: false,
});

const jetBrainsMono = JetBrains_Mono({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-jetbrains-mono",
  weight: "400",
  adjustFontFallback: false,
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className={`${rethinkSans.variable} ${jetBrainsMono.variable}`}
    >
      <head>
        <link rel="shortcut icon" href="/favicon.svg" type="image/svg+xml" />
        <link
          rel="preconnect"
          href="https://fonts.gstatic.com"
          crossOrigin=""
        />
      </head>
      <body>
        <SolanaWalletProvider>
          <ZplClientProvider>
            <BitcoinWalletProvider>
              <Theme appearance="dark">
                <GlobalModals />
                <div className="wrapper">
                  <Nav />
                  <div className="page-wrapper">{children}</div>
                </div>
                <ToastContainer
                  stacked
                  className="z-fubao-toast"
                  position="top-right"
                  autoClose={7500}
                  hideProgressBar={false}
                  rtl={false}
                  pauseOnFocusLoss
                  theme="dark"
                  pauseOnHover
                  transition={Slide}
                />
              </Theme>
            </BitcoinWalletProvider>
          </ZplClientProvider>
        </SolanaWalletProvider>
      </body>
    </html>
  );
}
