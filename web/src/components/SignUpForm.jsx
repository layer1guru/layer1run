import { useId } from 'react'

import { Button } from '@/components/Button'

export function SignUpForm() {
    let id = useId()

    return (
        <>
            <form className="relative isolate mt-8 flex items-center pr-1">
                <input
                    required
                    type="email"
                    autoComplete="email"
                    name="email"
                    id={id}
                    placeholder="curl -fsSL https://layer1.run/install | bash"
                    className="peer w-0 flex-auto bg-transparent px-4 py-2.5 text-base text-white placeholder:text-amber-200 tracking-wider focus:outline-none sm:text-[0.8125rem]/6"
                />

                <Button type="submit" arrow>
                    Copy
                </Button>

                <div className="absolute inset-0 -z-10 rounded-lg transition peer-focus:ring-4 peer-focus:ring-sky-300/15" />
                <div className="absolute inset-0 -z-10 rounded-lg bg-white/2.5 ring-1 ring-white/15 transition peer-focus:ring-sky-300" />
            </form>

            <small className="pl-3 pt-2 text-xs text-slate-100 italic tracking-wider">
                Paste into your macOS, Linux or WSL terminal
            </small>
        </>
    )
}
