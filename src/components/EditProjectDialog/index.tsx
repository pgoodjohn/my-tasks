import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from "../ui/button"
import { DialogClose } from "@radix-ui/react-dialog"
import { useForm } from "@tanstack/react-form"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { invoke_tauri_command } from "@/lib/utils"
import { Input } from "../ui/input"
import { useState } from "react"
import { Separator } from "../ui/separator"
import { Textarea } from "../ui/textarea"

interface EditProjectDialogProps {
    project: any,
}

const EditProjectDialog: React.FC<EditProjectDialogProps> = ({ project }) => {

    const [open, setOpen] = useState(false)

    const queryClient = useQueryClient()


    const editProjectForm = useForm({
        defaultValues: {
            id: project.id,
            title: project.title,
            description: project.description
        },
        onSubmit: async ({ value }) => {
            console.debug("Edit Project", value)
            await editProjectMutation.mutateAsync(value)
            queryClient.invalidateQueries({ queryKey: ['projects'] })
        }
    })

    const editProjectMutation = useMutation({
        mutationFn: async function (value: { id: string, title: string, description: string }) {
            invoke_tauri_command('update_project_command', { projectId: value.id, newTitle: value.title, newDescription: value.description })
        },
        onSuccess: () => {
            // Invalidate and refetch
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            editProjectForm.reset()
            setOpen(false)
        },
    })

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button>
                    Edit
                </Button>
            </DialogTrigger>
            <DialogContent>
                <form
                    onSubmit={(e) => {
                        e.preventDefault()
                        e.stopPropagation()
                        editProjectForm.handleSubmit()
                    }}
                >
                    <DialogHeader className="py-2">
                        <DialogTitle>Edit Project</DialogTitle>
                        <DialogDescription className="text-xs">
                            {project.id}
                        </DialogDescription>
                    </DialogHeader>
                    <Separator />
                    <div className="py-2">
                        <editProjectForm.Field
                            name="id"
                            children={(_field) => (
                                <></>
                            )} />
                        <editProjectForm.Field
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
                        <editProjectForm.Field
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

export default EditProjectDialog