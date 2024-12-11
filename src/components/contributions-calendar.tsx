import { ContributionCalendar } from 'react-contribution-calendar'

interface ContributionCalendarProps {
    variant?: string
}

const ContributionsCalendar: React.FC<ContributionCalendarProps> = ({ variant }) => {
    let start = undefined;
    let end = undefined;

    if (variant === 'month') {
        start = new Date()
        start.setDate(1)
        end = new Date()
        end.setMonth(start.getMonth() + 1);
        end.setDate(0);
    }

    return (
        <div>
            <h1>Contributions (WIP)</h1>
            <ContributionCalendar
                data={[]}
                daysOfTheWeek={['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']}
                // textColor="#1F2328"
                textColor="#FFF"
                start={start && start.toString()}
                end={end && end.toString()}
                startsOnSunday={true}
                includeBoundary={false}
                theme="grass"
                cx={10}
                cy={10}
                cr={2}
                scroll={false}
            />
        </div>
    );
}

export default ContributionsCalendar