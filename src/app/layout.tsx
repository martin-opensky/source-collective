import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import classNames from 'classnames'
import './globals.css'
import { ReduxProvider } from '@/store/Provider'
import DesktopNavigation from '@/components/Navigation/Desktop'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Source Collective',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body
        className={classNames({
          [inter.className]: true,
          'h-screen w-screen': true,
        })}
      >
        <ReduxProvider>
          <main className="flex min-h-full min-w-full flex-row">
            <DesktopNavigation />

            <section className="m-2 flex flex-1 flex-col items-start gap-2 lg:pl-16">
              {children}
            </section>
          </main>
        </ReduxProvider>
      </body>
    </html>
  )
}
