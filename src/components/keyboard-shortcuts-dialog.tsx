import { useEffect, useState } from "react"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
} from "@/components/ui/dialog"

export function KeyboardShortcutsDialog() {
    const [open, setOpen] = useState(false)

    useEffect(() => {
        const down = (e: KeyboardEvent) => {
            if (e.key === "h" && (e.metaKey || e.ctrlKey)) {
                e.preventDefault()
                setOpen((open) => !open)
            }
        }
        document.addEventListener("keydown", down)
        return () => document.removeEventListener("keydown", down)
    }, [])

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Keyboard Shortcuts</DialogTitle>
                </DialogHeader>
                <DialogDescription>
                    <p>cmd+b - toggle sidebar</p>
                    <p>cmd+h - open this dialog</p>
                </DialogDescription>
            </DialogContent>
        </Dialog>
    )
}
