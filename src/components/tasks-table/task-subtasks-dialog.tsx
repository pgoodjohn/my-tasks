import { Task } from "@/types";
import {
    Dialog,
    DialogContent,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useForm } from '@tanstack/react-form'
import {
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { DatePicker } from '@/components/datepicker';
import { toast } from "sonner";
import { DialogDescription } from "@radix-ui/react-dialog";
import { Separator } from "../ui/separator";
import { invoke_tauri_command } from '@/lib/utils';
import { ScrollArea } from "../ui/scroll-area";
import { SubtasksTable } from "../subtasks-table";

interface TaskSubtasksDialogProps {
    task: Task
}

export function TaskSubtasksDialog({ task }: TaskSubtasksDialogProps) {

    const queryClient = useQueryClient();

    return (
        <Dialog
            onOpenChange={() => queryClient.invalidateQueries({ queryKey: ['tasks'] })}
        >
            <DialogTrigger asChild>
                <Button className="w-full text-left" variant="ghost" size="xs">Subtasks</Button>
            </DialogTrigger>
            <DialogContent className="min-w-[720px]">
                <DialogHeader>
                    <DialogTitle>{task.title}</DialogTitle>
                    <DialogDescription>
                        Manage subtasks
                    </DialogDescription>
                </DialogHeader>
                <Separator />
                <CreateSubtaskForm parentTask={task} />
                <Separator />
                <ScrollArea className="max-h-[500px] pr-4">
                    <SubtasksTable task={task} />
                </ScrollArea>
            </DialogContent>
        </Dialog>
    )
}

interface CreateSubtaskFormProps {
    parentTask: Task
}

function CreateSubtaskForm({ parentTask }: CreateSubtaskFormProps) {

    const queryClient = useQueryClient()

    const newSubtaskMutation = useMutation({
        mutationFn: async ({ value }: { value: { parentTaskId: string, title: string, description: string, dueDate: Date | null } }) => {
            let res = await invoke_tauri_command('create_subtask_for_task_command', { parentTaskId: value.parentTaskId, title: value.title, description: value.description, due_date: value.dueDate })

            return res
        },
        onSuccess: () => {
            toast.success("Subtask created")
            queryClient.invalidateQueries({ queryKey: ['tasks', parentTask.id] })
            newSubtaskForm.reset()
        },
        onError: (error) => {
            console.error(error)
            // Handle the error, e.g., show an error message to the user or take other actions as needed.
            toast.error("Error creating subtask")
        }
    })

    const newSubtaskForm = useForm({
        defaultValues: {
            parentTaskId: parentTask.id,
            title: "",
            description: "",
            dueDate: null,
        },
        onSubmit: async (data) => {
            console.log("Submitting form", data)
            newSubtaskMutation.mutateAsync(data)
        }
    })

    return (
        <div>
            <form
                className='flex w-full items-center space-x-2'
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    newSubtaskForm.handleSubmit();
                }}
            >
                <div className='flex space-x-2'>
                    <newSubtaskForm.Field
                        name="title"
                        children={(field) => (
                            <Input
                                name={field.name}
                                value={field.state.value}
                                onBlur={field.handleBlur}
                                onChange={(e) => field.handleChange(e.target.value)}
                            />
                        )}
                    />
                    <newSubtaskForm.Field
                        name="dueDate"
                        children={(field) => (
                            <DatePicker
                                value={field.state.value}
                                onChange={field.handleChange}
                            />
                        )}
                    />
                </div>
                <Button type="submit">Submit</Button>
            </form>
        </div>
    )
}