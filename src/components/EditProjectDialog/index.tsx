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

interface EditProjectDialogProps {
    project: any,
}

const EditProjectDialog: React.FC<EditProjectDialogProps> = ({ project }) => {

    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button>
                    Edit
                </Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Edit Project</DialogTitle>
                    <DialogDescription className="text-xs">
                        {project.id}
                    </DialogDescription>
                </DialogHeader>
                <div>
                    <p>Project Title: {project.title}</p>
                </div>
                <div>
                    <p>Project Description: {project.description}</p>
                </div>
                <DialogFooter>
                    <DialogClose asChild>
                        <Button variant="outline">Cancel</Button>
                    </DialogClose>
                    <Button>Save</Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}

export default EditProjectDialog