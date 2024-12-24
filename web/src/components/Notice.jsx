'use client'

import { useRef, useState } from 'react'
import { Dialog, DialogBackdrop, DialogPanel, DialogTitle } from '@headlessui/react'

import Image from 'next/image'
import Link from 'next/link'

// import { CheckIcon } from '@heroicons/react/24/outline'
import { ArrowTopRightOnSquareIcon } from '@heroicons/react/24/outline'
import { XCircleIcon } from '@heroicons/react/24/outline'
import avaxLogo from '@/images/logos/avax.svg'

import videojs from 'video.js'
import VideoJS from '@/components/VideoPlayer'

export function NodeRunr() {
    const [open, setOpen] = useState(true)

    const playerRef = useRef(null)

    const videoForDesktop = {
        controls: true,
        autoplay: false,
        width: 400,
        sources: [{
            src: 'https://ik.imagekit.io/ikmedia/sample-video.mp4/ik-master.m3u8?tr=sr-240_360_480_720',
            type: 'application/x-mpegURL',
        }],
        plugins: {
            // httpSourceSelector: { default: 'auto' },
        },
    }

    const videoForMobile = {
        controls: true,
        autoplay: false,
        width: 370,
        sources: [{
            src: 'https://ik.imagekit.io/ikmedia/sample-video.mp4/ik-master.m3u8?tr=sr-240_360_480_720',
            type: 'application/x-mpegURL',
        }],
        plugins: {
            // httpSourceSelector: { default: 'auto' },
        },
    }

    const handlePlayerReady = (player) => {
        playerRef.current = player

        console.log(player.qualityLevels())

        // You can handle player events here, for example:
        player.on('waiting', () => {
            videojs.log('player is waiting')
        })

        player.on('dispose', () => {
            videojs.log('player will dispose')
        })
    }

    return (
        <Dialog open={open} onClose={setOpen} className="relative z-50">
            <DialogBackdrop transition className="fixed inset-0 bg-gray-500/85 transition-opacity data-[closed]:opacity-0 data-[enter]:duration-300 data-[leave]:duration-200 data-[enter]:ease-out data-[leave]:ease-in" />

            <div className="fixed inset-0 z-10 w-screen overflow-y-auto">
                <div className="flex min-h-full pb-20 items-end justify-center p-4 text-center sm:mb-0 sm:items-center sm:p-0">
                    <DialogPanel
                        transition
                        className="relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all data-[closed]:translate-y-4 data-[closed]:opacity-0 data-[enter]:duration-300 data-[leave]:duration-200 data-[enter]:ease-out data-[leave]:ease-in sm:my-8 sm:w-full sm:max-w-md sm:p-6 data-[closed]:sm:translate-y-0 data-[closed]:sm:scale-95"
                    >
                        <div className="pt-5">
                            <div className="flex justify-end">
                                <XCircleIcon
                                    onClick={() => setOpen(false)}
                                    className="absolute -mt-7 -mr-1 size-12 text-gray-500"
                                />
                            </div>

                            <div className="mx-auto flex size-16 items-center justify-center rounded-full bg-rose-50">
                                <Image
                                    src={avaxLogo}
                                    width={20}
                                    height={20}
                                    alt="Avalanche logo"
                                    className="size-10 text-green-600"
                                />
                            </div>

                            <div className="mt-3 text-center sm:mt-5">
                                <DialogTitle as="h3" className="text-3xl font-semibold text-slate-600">
                                    Avalanche <span className="text-4xl font-bold">Retro<i>9000</i></span>
                                    <br />Submission
                                </DialogTitle>

                                <div className="mt-2">
                                    <p className="py-2 px-5 text-base text-slate-600 tracking-tight text-justify">
                                        <span className="font-bold">Did you know — </span>
                                        the Avalanche team actively supports the hard-work of their ecosystem #BUIDLers with retro funding?
                                    </p>
                                </div>

                                <div className="flex justify-center sm:hidden border border-sky-500 rounded overflow-hidden">
                                    <VideoJS
                                        options={videoForMobile}
                                        onReady={handlePlayerReady}
                                    />
                                </div>
                                <div className="hidden sm:flex justify-center border-2 border-sky-500 rounded overflow-hidden">
                                    <VideoJS
                                        options={videoForDesktop}
                                        onReady={handlePlayerReady}
                                    />
                                </div>

                                <small className="block mt-1 text-slate-600 italic text-center tracking-wider">
                                    Watch a demo of NodΞRunr in action <span className="text-base">👀</span>
                                </small>
                            </div>
                        </div>

                        <div className="mt-5 sm:mt-6 flex flex-row gap-3">
                            <Link
                                href="https://canvas.layer1.run"
                                target="_blank"
                                className="inline-flex w-full items-center justify-center rounded-md bg-sky-600 px-3 py-2 text-lg font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
                            >
                                Lean Canvas
                                <ArrowTopRightOnSquareIcon className="pl-1 size-5 text-sky-200" />
                            </Link>

                            <Link
                                href="https://docs.layer1.run"
                                target="_blank"
                                className="inline-flex w-full items-center justify-center rounded-md bg-sky-600 px-3 py-2 text-lg font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
                            >
                                ReadTheDocs
                                <ArrowTopRightOnSquareIcon className="pl-1 size-5 text-sky-200" />
                            </Link>
                        </div>

                        <div className="mt-5 sm:mt-6">
                            <Link
                                href="https://retro9000.avax.network"
                                target="_blank"
                                className="inline-flex w-full items-center justify-center rounded-md bg-sky-600 px-3 py-2 text-2xl font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
                            >
                                Show Us <span className="px-2 text-rose-500 text-3xl">❤︎</span> By Voting
                                <ArrowTopRightOnSquareIcon className="pl-1 size-6 text-sky-200" />
                            </Link>
                        </div>
                    </DialogPanel>
                </div>
            </div>
        </Dialog>
    );
}
