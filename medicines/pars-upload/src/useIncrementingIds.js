import { useRef } from 'react'

export const useIncrementingIds = () => {
  const ref = useRef(0)

  return () => {
    ref.current += 1
    return ref.current
  }
}
