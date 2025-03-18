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
import type { Project } from "@/types"
import { EmojiPickerFormItem } from "@/components/emoji-picker-form-item"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { invoke_tauri_command } from "@/lib/utils"

interface EditProjectDialogProps {
    project: Project,
}

const EditProjectDialog: React.FC<EditProjectDialogProps> = ({ project }) => {

    const [open, setOpen] = useState(false)

    const queryClient = useQueryClient()


    const editProjectForm = useForm({
        defaultValues: {
            id: project.id,
            title: project.title,
            emoji: project.emoji,
            color: project.color,
            description: project.description
        },
        onSubmit: async ({ value }) => {
            await editProjectMutation.mutateAsync({
                ...value,
                emoji: value.emoji || '',
                color: value.color || '',
                description: value.description || ''
            })
            queryClient.invalidateQueries({ queryKey: ['projects'] })
        }
    })

    const editProjectMutation = useMutation({
        mutationFn: async function (value: { id: string, title: string, emoji: string, color: string, description: string }) {
            invoke_tauri_command('update_project_command', { projectId: value.id, title: value.title, emoji: value.emoji, color: value.color, description: value.description })
        },
        onSuccess: () => {
            // Invalidate and refetch
            toast.success(`Project "${editProjectForm.getFieldValue("title")}" was updated`)
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            editProjectForm.reset()
            setOpen(false)
        },
    })

    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
                <Button variant="outline">
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
                        <div className="flex container p-2">
                            <editProjectForm.Field
                                name="emoji"
                                children={(field) => {
                                    return (
                                        <div className="pr-2">
                                            <EmojiPickerFormItem value={field.state.value || ''} onSelect={(value) => field.setValue(value)} />
                                        </div>
                                    )
                                }}
                            />
                            <editProjectForm.Field
                                name="title"
                                children={(field) => (
                                    <div className="container pr-2">
                                        <Input
                                            name={field.name}
                                            value={field.state.value || ''}
                                            onBlur={field.handleBlur}
                                            onChange={(e) => field.setValue(e.target.value)}
                                        />
                                    </div>
                                )}
                            />
                            <editProjectForm.Field
                                name="color"
                                children={(field) => {
                                    return <ProjectColorCombobox selectedValue={field.state.value || ''} onChange={field.handleChange} />
                                }} />
                        </div>
                        <editProjectForm.Field
                            name="description"
                            children={(field) => (
                                <div className="p-2">
                                    <Textarea
                                        name={field.name}
                                        value={field.state.value || ''}
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