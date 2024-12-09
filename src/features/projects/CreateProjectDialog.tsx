import {
    Dialog,
    DialogContent,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from "../../components/ui/button"
import { DialogClose } from "@radix-ui/react-dialog"
import { useForm } from "@tanstack/react-form"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { invoke_tauri_command } from "@/lib/utils"
import { Input } from "../../components/ui/input"
import { useState } from "react"
import { Separator } from "../../components/ui/separator"
import { Textarea } from "../../components/ui/textarea"
import ProjectColorCombobox from "./project-color-combobox"
import { toast } from "sonner"


interface EditProjectDialogProps {
}

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
            console.debug("Edit Project", value)
            await createProjectFormMutation.mutateAsync(value)
            queryClient.invalidateQueries({ queryKey: ['projects'] })
        }
    })

    const createProjectFormMutation = useMutation({
        mutationFn: async function (value: { title: string, emoji: string, color: string, description: string }) {
            invoke_tauri_command('create_project_command', { title: value.title, emoji: value.emoji, color: value.color, description: value.description })
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
                        <div className="flex items-center p-2">
                            <createProjectForm.Field
                                name="emoji"
                                children={(field) => {
                                    return (
                                        <Input
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.setValue(e.target.value)}
                                        />
                                    )
                                }}
                            />
                            <createProjectForm.Field
                                name="color"
                                children={(field) => {
                                    return <ProjectColorCombobox selectedValue={field.state.value} onChange={field.handleChange} />
                                }} />
                        </div>
                        <createProjectForm.Field
                            name="title"
                            children={(field) => (
                                <div className="p-2">
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