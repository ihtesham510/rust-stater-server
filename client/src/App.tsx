import useTasks from './Hooks/useTasks'

function App() {
	const { tasks, deleteTask, addTask } = useTasks()
	return (
		<div className='h-max bg-gray-950 absolute top-0 w-full'>
			<hr />
			{tasks.data && tasks.data.length === 0 && (
				<div className='text-white text-3xl'>Empty tasks</div>
			)}
			{tasks.data &&
				tasks.data.map(task => (
					<div key={task.id} className='h-auto w-[500px] p-10 bg-gray-900 m-4'>
						<h1 className='text-3xl font-extrabold text-white'>{task.title}</h1>
						<p className='text-md font-semibold text-gray-400'>
							{task.description}
						</p>
						<button
							className='p-3 m-2 bg-white text-black rounded-md'
							onClick={() => {
								deleteTask(task.id)
							}}
						>
							delete
						</button>
					</div>
				))}
		</div>
	)
}

export default App
