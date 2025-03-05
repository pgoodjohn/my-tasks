import { useState } from "react"
import { useQuery, useQueryClient, useMutation } from "@tanstack/react-query"
import { CalendarIcon } from "lucide-react"
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
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover"
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

    const { data: recurringTask } = useQuery<RecurringTask>({
        queryKey: ["recurring-task", task.id],
        queryFn: async () => {
            const result = await invoke_tauri_command("get_recurring_task_command", { taskId: task.id })
            return result
        }
    })

    const form = useForm({
        defaultValues: {
            frequency: recurringTask?.frequency || Frequency.Weekly,
            interval: recurringTask?.interval || 1,
            firstDueDate: recurringTask ? new Date(recurringTask.next_due_at_utc) : new Date(),
        },
        onSubmit: async ({ value }) => {
            await setupRecurringTaskMutation.mutateAsync({
                taskId: task.id,
                frequency: value.frequency,
                interval: value.interval,
                firstDueDate: value.firstDueDate.toISOString()
            })
        }
    })

    const setupRecurringTaskMutation = useMutation({
        mutationFn: async (data: { taskId: string, frequency: Frequency, interval: number, firstDueDate: string }) => {
            return invoke_tauri_command("setup_recurring_task_command", { data: data })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["recurring-task", task.id] })
            toast.success("Recurring task settings updated")
            form.reset()
            setOpen(false)
        },
        onError: (error) => {
            console.error("Failed to setup recurring task:", error)
            toast.error("Failed to update recurring task settings")
        }
    })

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button variant="outline" size="sm">
                    {recurringTask ? "Edit Recurring" : "Make Recurring"}
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
                        <DialogTitle>Recurring Task Settings</DialogTitle>
                        <DialogDescription>
                            Configure how often this task should repeat after completion.
                        </DialogDescription>
                    </DialogHeader>
                    <Separator />
                    <div className="py-4 space-y-4">
                        <form.Field
                            name="frequency"
                            children={(field) => (
                                <div className="space-y-2">
                                    <label className="text-sm font-medium">Frequency</label>
                                    <select
                                        className="w-full p-2 border rounded-md"
                                        value={field.state.value}
                                        onChange={(e) => field.setValue(e.target.value as Frequency)}
                                    >
                                        <option value={Frequency.Daily}>Daily</option>
                                        <option value={Frequency.Weekly}>Weekly</option>
                                        <option value={Frequency.Monthly}>Monthly</option>
                                        <option value={Frequency.Yearly}>Yearly</option>
                                    </select>
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
                        <form.Field
                            name="firstDueDate"
                            children={(field) => (
                                <div className="space-y-2">
                                    <label className="text-sm font-medium">First Due Date</label>
                                    <Popover>
                                        <PopoverTrigger asChild>
                                            <Button
                                                variant="outline"
                                                className="w-full justify-start text-left font-normal"
                                            >
                                                {field.state.value ? (
                                                    format(field.state.value, "PPP")
                                                ) : (
                                                    <span>Pick a date</span>
                                                )}
                                                <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                                            </Button>
                                        </PopoverTrigger>
                                        <PopoverContent className="w-auto p-0" align="start">
                                            <Calendar
                                                mode="single"
                                                selected={field.state.value}
                                                onSelect={(date) => date && field.setValue(date)}
                                                disabled={(date) =>
                                                    date < new Date()
                                                }
                                                initialFocus
                                            />
                                        </PopoverContent>
                                    </Popover>
                                </div>
                            )}
                        />
                    </div>
                    <Separator />
                    <DialogFooter>
                        <div className="py-2">
                            <DialogClose asChild>
                                <Button className="m-2" variant="outline">Cancel</Button>
                            </DialogClose>
                            <Button className="m-2" type="submit">Save</Button>
                        </div>
                    </DialogFooter>
                </form>
            </DialogContent>
        </Dialog>
    )
} 