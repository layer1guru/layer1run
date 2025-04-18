import Script from 'next/script'

import { Inter } from 'next/font/google'
import localFont from 'next/font/local'
import clsx from 'clsx'

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
                <Providers>
                    {children}
                </Providers>
            </body>
            <Script src="https://scripts.simpleanalyticscdn.com/latest.js" />
        </html>
    )
}
