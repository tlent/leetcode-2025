import { expect, test } from 'bun:test';
import { MyQueue } from './n0232_implement_queue_using_stacks';

test('MyQueue', () => {
  const queue = new MyQueue();
  queue.push(1);
  queue.push(2);
  expect(queue.peek()).toEqual(1);
  expect(queue.pop()).toEqual(1);
  expect(queue.empty()).toEqual(false);
});
