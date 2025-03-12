import { useState, useEffect } from "react"
import { useQuery, useQueryClient, useMutation } from "@tanstack/react-query"
import { CalendarIcon, Loader2 } from "lucide-react"
import { format } from "date-fns"
import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
    DialogClose
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select"
import { Separator } from "@/components/ui/separator"
import { useForm } from "@tanstack/react-form"
import { invoke_tauri_command } from "@/lib/utils"
import type { Task, RecurringTask } from "@/types"
import { Frequency } from "@/types"
import { toast } from "sonner"

interface RecurringTaskDialogProps {
    task: Task
}

export function RecurringTaskDialog({ task }: RecurringTaskDialogProps) {
    const [open, setOpen] = useState(false)
    const queryClient = useQueryClient()

    const { data: recurringTask, isLoading } = useQuery<RecurringTask>({
        queryKey: ["recurring-task", task.id],
        queryFn: async () => {
            const result = await invoke_tauri_command("get_recurring_task_command", { taskId: task.id })
            return result
        }
    })

    const form = useForm({
        defaultValues: {
            frequency: Frequency.Weekly,
            interval: 1,
        },
        onSubmit: async ({ value }) => {
            if (recurringTask) {
                await updateRecurringTaskMutation.mutateAsync({
                    taskId: task.id,
                    frequency: value.frequency,
                    interval: value.interval,
                })
            } else {
                await setupRecurringTaskMutation.mutateAsync({
                    taskId: task.id,
                    frequency: value.frequency,
                    interval: value.interval,
                })
            }
        }
    })

    // Update form values when recurringTask changes
    useEffect(() => {
        if (recurringTask) {
            form.setFieldValue("frequency", recurringTask.frequency)
            form.setFieldValue("interval", recurringTask.interval)
        }
    }, [recurringTask, form])

    const setupRecurringTaskMutation = useMutation({
        mutationFn: async (data: { taskId: string, frequency: Frequency, interval: number }) => {
            return invoke_tauri_command("setup_recurring_task_command", { data: data })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["recurring-task", task.id] })
            toast.success("Recurring task settings created")
            form.reset()
            setOpen(false)
        },
        onError: (error) => {
            console.error("Failed to setup recurring task:", error)
            toast.error("Failed to create recurring task settings")
        }
    })

    const updateRecurringTaskMutation = useMutation({
        mutationFn: async (data: { taskId: string, frequency: Frequency, interval: number }) => {
            return invoke_tauri_command("update_recurring_task_command", { data: data })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["recurring-task", task.id] })
            toast.success("Recurring task settings updated")
            form.reset()
            setOpen(false)
        },
        onError: (error) => {
            console.error("Failed to update recurring task:", error)
            toast.error("Failed to update recurring task settings")
        }
    })

    const deleteRecurringTaskMutation = useMutation({
        mutationFn: async () => {
            return invoke_tauri_command("delete_recurring_task_command", { taskId: task.id })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["recurring-task", task.id] })
            toast.success("Recurring settings removed")
            setOpen(false)
        },
        onError: (error) => {
            console.error("Failed to delete recurring task:", error)
            toast.error("Failed to remove recurring settings")
        }
    })

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
                <form
                    onSubmit={(e) => {
                        e.preventDefault()
                        e.stopPropagation()
                        form.handleSubmit()
                    }}
                >
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
                                {recurringTask && (
                                    <Button
                                        type="button"
                                        variant="destructive"
                                        onClick={() => deleteRecurringTaskMutation.mutate()}
                                    >
                                        Remove Recurring
                                    </Button>
                                )}
                            </div>
                            <div className="flex gap-2">
                                <DialogClose asChild>
                                    <Button variant="outline">Cancel</Button>
                                </DialogClose>
                                <Button type="submit">
                                    {recurringTask ? "Update" : "Save"}
                                </Button>
                            </div>
                        </div>
                    </DialogFooter>
                </form>
            </DialogContent>
        </Dialog>
    )
} 