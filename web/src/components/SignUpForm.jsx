import { useId } from 'react'

import { Button } from '@/components/Button'

export function SignUpForm() {
    let id = useId()

    return (
        <>
            <div className="relative isolate mt-8 flex items-center pr-1">
                <input
                    id={id}
                    value="curl -sSL https://layer1.run/linux | bash"
                    className="peer w-0 flex-auto bg-transparent px-4 py-2.5 text-xs text-amber-200 tracking-wider focus:outline-none sm:text-[0.85rem]"
                />

                <Button arrow>
                    Copy
                </Button>

                <div className="absolute inset-0 -z-10 rounded-lg transition peer-focus:ring-4 peer-focus:ring-sky-300/15" />
                <div className="absolute inset-0 -z-10 rounded-lg bg-white/2.5 ring-1 ring-white/15 transition peer-focus:ring-sky-300" />
            </div>

            <small className="pl-3 pt-2 text-xs text-slate-100 italic tracking-wider">
                paste into your macOS, Linux or WSL terminal
            </small>
        </>
    )
}
