import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import axios from 'axios'
import { Task } from '../utils/types'

export default function useTasks() {
	let queryclient = useQueryClient()
	const tasks = useQuery({
		queryFn: async () => {
			let res = await axios.get<Task[]>('/api/tasks')
			return res.data
		},
		queryKey: ['tasks'],
	})
	const taskMutation = useMutation({
		mutationFn: async (task: Task) => {
			let res = await axios.post('/api/tasks', task)
			return res.data
		},
		onSuccess() {
			queryclient.refetchQueries({ queryKey: ['tasks'] })
		},
	})
	const deleteTaskMutation = useMutation({
		mutationFn: async (id: number) => {
			let res = await axios.delete(`/api/tasks/${id}`)
			return res.data
		},
		onSuccess() {
			queryclient.refetchQueries({ queryKey: ['tasks'] })
		},
	})
	const addTask = taskMutation.mutate
	const deleteTask = deleteTaskMutation.mutate
	return { tasks, addTask, deleteTask }
}
