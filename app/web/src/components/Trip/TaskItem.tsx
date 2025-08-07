import { useEffect, useRef, useState } from 'react';
import { IconDotsVertical, IconTrash } from '@tabler/icons-react';
import { Checkbox, Group, Menu, Textarea, UnstyledButton } from '@mantine/core';
import { Task } from '@/models/tasks';
import { useReplicache } from '@/providers/ReplicacheProvider';

export const TaskItem = ({ task }: { task: Task }) => {
  const { rep } = useReplicache();
  const [description, setDescription] = useState(task.description);
  const [descriptionLimitReached, setDescriptionLimitReached] = useState(false);

  useEffect(() => {
    setDescription(task.description);
  }, [task.description]);

  const setTaskDescription = (description: string) => {
    if (!rep) return;

    rep.mutate.updateTask({
      id: task.id,
      description,
    });
  };

  const deleteTask = () => {
    if (!rep) return;
    rep.mutate.deleteTask(task.id);
  };

  const setTaskCompleted = (completed: boolean) => {
    if (!rep) return;
    rep.mutate.updateTask({
      id: task.id,
      completed,
    });
  };

  const setTaskUrgency = (urgency: 'high' | 'medium' | 'low') => {
    if (!rep) return;
    rep.mutate.updateTask({
      id: task.id,
      urgency,
    });
  };

  return (
    <Group>
      <Checkbox checked={task.completed} onChange={() => setTaskCompleted(!task.completed)} />
      <Textarea
        variant="unstyled"
        w={{ base: '89%', lg: '77%' }}
        value={description}
        onKeyDown={(e) => {
          if (e.key === 'Enter') {
            e.preventDefault();
            setTaskDescription(description.trim());
            setDescriptionLimitReached(false);
            e.currentTarget.blur();
          }
        }}
        onBlur={() => {
          setDescription(task.description ?? '');
          setDescriptionLimitReached(false);
        }}
        onChange={(e) => {
          if (e.target.value.length > 100) {
            setDescriptionLimitReached(true);
          } else {
            setDescriptionLimitReached(false);
            setDescription(e.target.value);
          }
        }}
        placeholder="New task..."
        autosize
        error={descriptionLimitReached ? 'Description limit reached' : false}
      />
      <Menu position="bottom-end">
        <Menu.Target>
          <UnstyledButton>
            <IconDotsVertical size={16} />
          </UnstyledButton>
        </Menu.Target>
        <Menu.Dropdown>
          <Menu.Label>Urgency</Menu.Label>
          <Menu.Item color="red" onClick={() => setTaskUrgency('high')}>
            High
          </Menu.Item>
          <Menu.Item color="yellow" onClick={() => setTaskUrgency('medium')}>
            Medium
          </Menu.Item>
          <Menu.Item color="green" onClick={() => setTaskUrgency('low')}>
            Low
          </Menu.Item>
          <Menu.Divider />
          <Menu.Item leftSection={<IconTrash size={16} />} color="red" onClick={deleteTask}>
            Delete
          </Menu.Item>
        </Menu.Dropdown>
      </Menu>
    </Group>
  );
};
