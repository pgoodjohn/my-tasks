import { invoke_tauri_command } from '@/lib/utils';
import { useQuery } from '@tanstack/react-query';
import { ContributionCalendar } from 'react-contribution-calendar'

interface ContributionCalendarProps {
    variant?: string
}

const ContributionsCalendar: React.FC<ContributionCalendarProps> = ({ }) => {

    const contributionsCalendarDataQuery = useQuery({
        queryKey: ['tasks', 'completed'],
        queryFn: async () => {
            return invoke_tauri_command('load_task_activity_statistics_command', {})
        }
    })

    if (contributionsCalendarDataQuery.isLoading) {
        return <div>Loading...</div>
    }

    if (contributionsCalendarDataQuery.isError) {
        return <div>Error loading contributions</div>
    }

    console.log(contributionsCalendarDataQuery.data)

    return (
        <div>
            <ContributionCalendar
                data={contributionsCalendarDataQuery.data}
                daysOfTheWeek={['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']}
                end={new Date().toISOString().split('T')[0]}
                startsOnSunday={true}
                includeBoundary={false}
                scroll={false}
                hideDayLabels={true}
                hideMonthLabels={true}
                hideDescription={true}
            />
        </div>
    );
}

export default ContributionsCalendar