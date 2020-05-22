import { useState } from 'react'

export const Wizard = ({
  initialSteps,
  success: Success,
  onComplete,
  extraProps,
}) => {
  const [steps, setSteps] = useState(() =>
    initialSteps.map(({ type, component }) => ({ type, component, data: null }))
  )
  const [pageIndex, setPageIndex] = useState(0)

  const currentPage = steps[pageIndex]
  const Component = currentPage.component

  const setPageData = (pageData) =>
    setSteps((steps) =>
      steps.map((page, i) =>
        i === pageIndex ? { ...page, data: pageData } : page
      )
    )

  const onSubmit = (formDataForPage) => {
    setPageData(formDataForPage)
    const newIndex = pageIndex + 1
    if (newIndex < steps.length) {
      setPageIndex(newIndex)
    } else {
      onComplete(steps)
    }
  }

  const onRepeatPage = (formData) => {
    setSteps((steps) => {
      const newSteps = [...steps]
      newSteps.splice(pageIndex, 0, {
        ...currentPage,
        data: null,
      })
      return newSteps
    })
    onSubmit(formData)
  }

  const goBack = () => {
    setPageIndex((i) => i - 1)
  }

  if (pageIndex >= steps.length) {
    return <Success />
  }

  return (
    <Component
      key={pageIndex}
      currentStepData={currentPage.data}
      currentStepIndex={pageIndex}
      steps={steps.map((step, index) => ({ ...step, index }))}
      submit={onSubmit}
      repeatPage={onRepeatPage}
      savePageState={setPageData}
      goBack={goBack}
      goToPage={(index) => setPageIndex(index)}
      {...extraProps}
    />
  )
}
