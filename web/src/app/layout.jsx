import { Inter } from 'next/font/google'
import localFont from 'next/font/local'
import clsx from 'clsx'

import PlausibleProvider from 'next-plausible'
import { Providers } from '@/app/providers'

import '@/styles/tailwind.css'

const inter = Inter({
    subsets: ['latin'],
    display: 'swap',
    variable: '--font-inter',
})

const monaSans = localFont({
    src: '../fonts/Mona-Sans.var.woff2',
    display: 'swap',
    variable: '--font-mona-sans',
    weight: '200 900',
})

export const metadata = {
    title: 'NodΞRunr by L1 GÜRŲ',
    description:
        'NodΞRunr is a lightweight, smart daemon delivering effortless SysOps to Avalanche Founders and Teams.',
}

export default function RootLayout({ children }) {
    return (
        <html
            lang="en"
            className={clsx('h-full antialiased', inter.variable, monaSans.variable)}
            suppressHydrationWarning
        >
            <body className="flex min-h-full flex-col bg-white dark:bg-gray-950">
                <PlausibleProvider
                    domain="layer1.run"
                    customDomain="https://plausible.layer1.guru"
                >
                    <Providers>
                        {children}
                    </Providers>
                </PlausibleProvider>
            </body>
        </html>
    )
}
