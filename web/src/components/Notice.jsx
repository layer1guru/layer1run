'use client'

import { useRef, useState } from 'react'
import { Dialog, DialogBackdrop, DialogPanel, DialogTitle } from '@headlessui/react'

import Image from 'next/image'
import Link from 'next/link'

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
            // src: 'https://ik.imagekit.io/ikmedia/sample-video.mp4/ik-master.m3u8?tr=sr-240_360_480_720',
            // type: 'application/x-mpegURL',
            src: 'https://media.layer1.guru/vid/retro9000-intro.mp4',
            type: 'video/mp4',
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
            src: 'https://media.layer1.guru/vid/retro9000-intro.mp4',
            type: 'video/mp4',
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
                        className="relative transform overflow-hidden rounded-lg bg-gradient-to-b from-red-100 to-red-50 px-4 pb-4 pt-5 text-left shadow-xl transition-all data-[closed]:translate-y-4 data-[closed]:opacity-0 data-[enter]:duration-300 data-[leave]:duration-200 data-[enter]:ease-out data-[leave]:ease-in sm:my-8 sm:w-full sm:max-w-md sm:p-6 data-[closed]:sm:translate-y-0 data-[closed]:sm:scale-95"
                    >
                        <div className="pt-2">
                            <div className="flex justify-end">
                                <XCircleIcon
                                    onClick={() => setOpen(false)}
                                    className="absolute -mt-4 -mr-1 size-12 text-gray-500 opacity-50 cursor-pointer"
                                />
                            </div>

                            <div className="mx-auto flex size-14 items-center justify-center rounded-full bg-rose-700/20">
                                <Image
                                    src={avaxLogo}
                                    width={0}
                                    height={0}
                                    alt="Avalanche logo"
                                    className="size-8"
                                />
                            </div>

                            <div className="mt-3 text-center sm:mt-5">
                                <DialogTitle as="h3" className="text-3xl font-semibold text-slate-600">
                                    Avalanche <span className="text-4xl font-bold">Retro<i>9000</i></span>
                                    <br />Submission
                                </DialogTitle>

                                <div className="my-2">
                                    <p className="py-2 px-5 text-base text-slate-600 tracking-tight text-justify">
                                        <span className="font-bold">Did you know — </span>
                                        the Avalanche team is actively supporting the hard-work of their ecosystem #BUIDLers with retro funding?
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

                                <small className="flex justify-center items-center mt-0 text-slate-600 font-bold italic text-center tracking-widest">
                                    Watch a demo of NodΞRunr in action <span className="pl-1 text-lg">👀</span>
                                </small>
                            </div>
                        </div>

                        <div className="mt-3 sm:mt-2 flex flex-row gap-3">
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

                        <div className="mt-3">
                            <Link
                                href="https://retro9000.avax.network/discover-builders/cm490lkn9004bbuff1hmdkj8r"
                                target="_blank"
                                className="inline-flex w-full items-center justify-center rounded-md bg-sky-600 px-3 py-2 text-2xl font-semibold text-white shadow-sm hover:bg-sky-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-sky-600"
                            >
                                Show Ur <span className="px-2 text-rose-500 text-4xl">❤︎</span> By Voting
                                <ArrowTopRightOnSquareIcon className="pl-2 size-7 text-sky-200" />
                            </Link>
                        </div>
                    </DialogPanel>
                </div>
            </div>
        </Dialog>
    );
}
