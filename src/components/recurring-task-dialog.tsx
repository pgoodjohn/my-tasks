import { useState } from "react"
import { useQuery, useQueryClient, useMutation } from "@tanstack/react-query"
import { Loader2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select"
import { Input } from "@/components/ui/input"
import { Separator } from "@/components/ui/separator"
import { useForm } from "@tanstack/react-form"
import { invoke_tauri_command } from "@/lib/utils"
import type { Task, RecurringTask } from "@/types"
import { Frequency } from "@/types"
import { toast } from "sonner"

interface RecurringTaskFormProps {
    taskId: string
    defaultValues: {
        frequency: Frequency
        interval: number
    }
    onSubmit: (values: { frequency: Frequency; interval: number }) => Promise<void>
    onCancel: () => void
    onDelete?: () => void
    submitLabel: string
}

function RecurringTaskForm({ taskId, defaultValues, onSubmit, onCancel, onDelete, submitLabel }: RecurringTaskFormProps) {
    const form = useForm({
        defaultValues,
        onSubmit: async ({ value }) => {
            await onSubmit(value)
        }
    })

    return (
        <form onSubmit={(e) => {
            e.preventDefault()
            e.stopPropagation()
            form.handleSubmit()
        }}>
            <div className="py-4 space-y-4">
                <form.Field
                    name="frequency"
                    children={(field) => (
                        <div className="space-y-2">
                            <label className="text-sm font-medium">Frequency</label>
                            <Select
                                value={field.state.value}
                                onValueChange={(value) => field.setValue(value as Frequency)}
                            >
                                <SelectTrigger>
                                    <SelectValue placeholder="Select frequency" />
                                </SelectTrigger>
                                <SelectContent>
                                    <SelectItem value={Frequency.Daily}>Daily</SelectItem>
                                    <SelectItem value={Frequency.Weekly}>Weekly</SelectItem>
                                    <SelectItem value={Frequency.Monthly}>Monthly</SelectItem>
                                    <SelectItem value={Frequency.Yearly}>Yearly</SelectItem>
                                </SelectContent>
                            </Select>
                        </div>
                    )}
                />
                <form.Field
                    name="interval"
                    children={(field) => (
                        <div className="space-y-2">
                            <label className="text-sm font-medium">Interval</label>
                            <Input
                                type="number"
                                min={1}
                                max={365}
                                value={field.state.value}
                                onChange={(e) => field.setValue(parseInt(e.target.value))}
                            />
                        </div>
                    )}
                />
            </div>
            <Separator />
            <DialogFooter>
                <div className="flex justify-between w-full py-2">
                    <div>
                        {onDelete && (
                            <Button
                                type="button"
                                variant="destructive"
                                onClick={onDelete}
                            >
                                Remove Recurring
                            </Button>
                        )}
                    </div>
                    <div className="flex gap-2">
                        <Button type="button" variant="outline" onClick={onCancel}>
                            Cancel
                        </Button>
                        <Button type="submit">
                            {submitLabel}
                        </Button>
                    </div>
                </div>
            </DialogFooter>
        </form>
    )
}

interface RecurringTaskDialogProps {
    task: Task
}

export function RecurringTaskDialog({ task }: RecurringTaskDialogProps) {
    const [open, setOpen] = useState(false)
    const queryClient = useQueryClient()

    const { data: recurringTask, isLoading } = useQuery<RecurringTask>({
        queryKey: ["recurring-task", task.id],
        queryFn: async () => {
            return invoke_tauri_command("get_recurring_task_command", { taskId: task.id })
        }
    })

    const handleSubmit = async (values: { frequency: Frequency, interval: number }) => {
        const command = recurringTask ? "update_recurring_task_command" : "setup_recurring_task_command"
        const data = { taskId: task.id, ...values }

        try {
            await invoke_tauri_command(command, { data })
            await queryClient.invalidateQueries({ queryKey: ["recurring-task", task.id] })
            toast.success(recurringTask ? "Recurring task updated" : "Recurring task created")
            setOpen(false)
        } catch (error) {
            console.error(`Failed to ${recurringTask ? 'update' : 'create'} recurring task:`, error)
            toast.error(`Failed to ${recurringTask ? 'update' : 'create'} recurring task`)
        }
    }

    const handleDelete = async () => {
        try {
            await invoke_tauri_command("delete_recurring_task_command", { taskId: task.id })
            await queryClient.invalidateQueries({ queryKey: ["recurring-task", task.id] })
            toast.success("Recurring settings removed")
            setOpen(false)
        } catch (error) {
            console.error("Failed to delete recurring task:", error)
            toast.error("Failed to remove recurring settings")
        }
    }

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button variant="outline" size="sm">
                    {isLoading ? (
                        <Loader2 className="h-4 w-4 animate-spin" />
                    ) : recurringTask ? (
                        "Edit Recurring"
                    ) : (
                        "Make Recurring"
                    )}
                </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px]">
                <DialogHeader>
                    <DialogTitle>
                        {recurringTask ? "Edit Recurring Settings" : "Make Task Recurring"}
                    </DialogTitle>
                    <DialogDescription>
                        {recurringTask
                            ? "Modify how often this task should repeat after completion."
                            : "Configure how often this task should repeat after completion."
                        }
                    </DialogDescription>
                </DialogHeader>
                <Separator />
                <RecurringTaskForm
                    taskId={task.id}
                    defaultValues={{
                        frequency: recurringTask?.frequency ?? Frequency.Weekly,
                        interval: recurringTask?.interval ?? 1,
                    }}
                    onSubmit={handleSubmit}
                    onCancel={() => setOpen(false)}
                    onDelete={recurringTask ? handleDelete : undefined}
                    submitLabel={recurringTask ? "Update" : "Save"}
                />
            </DialogContent>
        </Dialog>
    )
} 