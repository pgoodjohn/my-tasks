import { Separator } from '@/components/ui/separator';
import { RollingWeekGraphs } from './rolling-week-graphs';
import ContributionsCalendar from '@/components/contributions-calendar';

export function RouteComponent() {

    return <div>
        <RollingWeekGraphs />
        <Separator className='my-4' />
        <div>
            <p>Completions Calendar</p>
            <ContributionsCalendar />
        </div>
    </div>
}