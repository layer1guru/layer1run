'use client'

import { useId } from 'react'

import { Button } from '@/components/Button'

const copyToClipboard = () => {
    /* Copy to clipboard. */
    navigator.clipboard.writeText('curl -sSL https://layer1.run/linux | bash')
        .then(() => console.log('Remote script command copied to clipboard!'))
        .catch((error) => console.error('Error copying to clipboard:', error))

    /* Notify the user. */
    alert(`
The setup command has been saved to your clipboard.
Now paste into your terminal (Ctrl + v) and go!`)
}

export function ScriptCommand() {
    let id = useId()

    return (
        <>
            <div className="relative isolate mt-8 flex items-center pr-1">
                <input
                    id={id}
                    defaultValue="curl -sSL https://layer1.run/linux | bash"
                    className="peer w-0 flex-auto bg-transparent px-4 py-2.5 text-xs text-amber-200 tracking-wider focus:outline-none sm:text-[0.85rem]"
                />

                <Button onClick={copyToClipboard} arrow>
                    Copy
                </Button>

                <div className="absolute inset-0 -z-10 rounded-lg transition peer-focus:ring-4 peer-focus:ring-sky-300/15" />
                <div className="absolute inset-0 -z-10 rounded-lg bg-white/2.5 ring-1 ring-white/15 transition peer-focus:ring-sky-300" />
            </div>

            <small className="pl-3 pt-2 text-xs text-slate-100 tracking-wider">
                ↑ paste into your Linux, <span className="line-through">macOS</span> or WSL terminal ↑
            </small>
        </>
    )
}
