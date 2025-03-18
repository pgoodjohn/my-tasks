import { DialogClose } from "@radix-ui/react-dialog"
import { useForm } from "@tanstack/react-form"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { useState } from "react"
import { toast } from "sonner"
import { Input } from "../../components/ui/input"
import { Separator } from "../../components/ui/separator"
import { Textarea } from "../../components/ui/textarea"
import { Button } from "../../components/ui/button"
import ProjectColorCombobox from "./project-color-combobox"
import { invoke_tauri_command } from "@/lib/utils"
import {
    Dialog,
    DialogContent,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { EmojiPickerFormItem } from "@/components/emoji-picker-form-item"


interface EditProjectDialogProps { }

const CreateProjectDialog: React.FC<EditProjectDialogProps> = ({ }) => {

    const [open, setOpen] = useState(false)

    const queryClient = useQueryClient()

    const createProjectForm = useForm({
        defaultValues: {
            title: "",
            emoji: "",
            color: "",
            description: ""
        },
        onSubmit: async ({ value }) => {
            await createProjectFormMutation.mutateAsync(value)
            queryClient.invalidateQueries({ queryKey: ['projects'] })
        }
    })

    const createProjectFormMutation = useMutation({
        mutationFn: async function (value: { title: string, emoji: string, color: string, description: string }) {
            return await invoke_tauri_command('create_project_command', { title: value.title, emoji: value.emoji, color: value.color, description: value.description })
        },
        onSuccess: () => {
            // Invalidate and refetch
            toast.success(`Project "${createProjectForm.getFieldValue("title")}" was created`)
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            createProjectForm.reset()
            setOpen(false)
        },
    })

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button variant="outline">
                    New Project
                </Button>
            </DialogTrigger>
            <DialogContent>
                <form
                    onSubmit={(e) => {
                        e.preventDefault()
                        e.stopPropagation()
                        createProjectForm.handleSubmit()
                    }}
                >
                    <DialogHeader className="py-2">
                        <DialogTitle>New Project</DialogTitle>
                    </DialogHeader>
                    <Separator />
                    <div className="py-2">
                        <div className="flex container p-2">
                            <createProjectForm.Field
                                name="emoji"
                                children={(field) => {
                                    return (
                                        <div className="pr-2">
                                            <EmojiPickerFormItem value={field.state.value} onSelect={(value) => field.setValue(value)} />
                                        </div>
                                    )
                                }}
                            />
                            <createProjectForm.Field
                                name="title"
                                children={(field) => (
                                    <div className="container pr-2">
                                        <Input
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.setValue(e.target.value)}
                                        />
                                    </div>
                                )}
                            />
                            <createProjectForm.Field
                                name="color"
                                children={(field) => {
                                    return <ProjectColorCombobox selectedValue={field.state.value} onChange={field.handleChange} />
                                }} />
                        </div>
                        <createProjectForm.Field
                            name="description"
                            children={(field) => (
                                <div className="p-2">
                                    <Textarea
                                        name={field.name}
                                        value={field.state.value}
                                        placeholder="Description"
                                        onBlur={field.handleBlur}
                                        onChange={(e) => field.setValue(e.target.value)}
                                    />
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

export default CreateProjectDialog