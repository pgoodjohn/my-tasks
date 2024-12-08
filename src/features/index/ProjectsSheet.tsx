import {
    Sheet,
    SheetContent,
    SheetDescription,
    SheetHeader,
    SheetTitle,
    SheetTrigger,
} from "@/components/ui/sheet"
import { Button } from "../../components/ui/button";
import {
    useQuery,
    useMutation,
    useQueryClient,
} from '@tanstack/react-query'
import { invoke } from '@tauri-apps/api/core';
import { ColumnDef } from "@tanstack/react-table"
import { Input } from '@/components/ui/input';
import { useForm } from '@tanstack/react-form'
import { DataTable } from "../../components/data-table";
import { invoke_tauri_command } from "../../lib/utils";
import { toast } from "sonner";

const ProjectsSheet = () => {

    return (
        <Sheet>
            <SheetTrigger>
                <Button variant={"outline"}>Projects</Button></SheetTrigger>
            <SheetContent>
                <SheetHeader>
                    <SheetTitle>Projects</SheetTitle>
                    <SheetDescription>
                        These are your projects.
                    </SheetDescription>
                </SheetHeader>
                <div>
                    <ProjectsList />
                    <NewProjectForm />
                </div>
            </SheetContent>
        </Sheet>

    )
}

export default ProjectsSheet;

const ProjectsList = () => {
    const todosListQuery = useQuery({
        queryKey: ['projects'],
        queryFn: async () => {
            let data = await invoke_tauri_command('load_projects_command', { showArchivedProjects: false })
            return data
        }
    })

    if (todosListQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (todosListQuery.isError) {
        return <div>Error loading tasks</div>
    }

    if (todosListQuery.data) {
        console.debug("Loaded Data", todosListQuery.data)
    }

    return (
        <div className='py-2'>
            {todosListQuery.data ? <DataTable data={todosListQuery.data} columns={columns} /> : <div>No Data</div>}
        </div>
    )
}

type Project = {
    id: string
    title: string
    description: string | null
    created_at_utc: string
    updated_at_utc: string
}

const columns: ColumnDef<Project>[] = [
    {
        accessorKey: "title",
        header: "Title",
    },
]


const NewProjectForm: React.FC = () => {
    const queryClient = useQueryClient()

    const mutation = useMutation({
        mutationFn: async function (value: { title: string, description: string }) {
            let res = await invoke('create_project_command', { title: value.title, description: value.description });
            console.debug("Save Rust Returned", res)
            return res
        },
        onSuccess: () => {
            // Invalidate and refetch
            toast.success(`Project "${todoForm.getFieldValue("title")}" created`)
            queryClient.invalidateQueries({ queryKey: ['projects'] })
            todoForm.reset()
        },
    })


    const todoForm = useForm({
        defaultValues: {
            title: '',
            description: '',
        },
        onSubmit: async ({ value }) => {
            // Do something with form data
            console.log(value)
            await mutation.mutateAsync(value)
        },
    })

    return (
        <div className='py-2 w-full'>
            <form
                className='flex w-full items-center space-x-2'
                onSubmit={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    todoForm.handleSubmit();
                }}
            >
                <div className='flex space-x-2'>
                    <todoForm.Field
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
                </div>
                <Button type="submit">Save</Button>
            </form>
        </div>
    )
}