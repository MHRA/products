import { useState } from 'react'
import { useIncrementingIds } from './useIncrementingIds'

export const Wizard = ({ initialSteps, onComplete, extraProps, flowName }) => {
  const getNextId = useIncrementingIds()
  const [steps, setSteps] = useState(() =>
    initialSteps.map(({ type, component }) => ({
      id: getNextId(),
      type,
      component,
      data: null,
    }))
  )
  const [pageIndex, setPageIndex] = useState(0)

  if (pageIndex >= steps.length) return null

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
      newSteps.splice(pageIndex + 1, 0, {
        ...currentPage,
        id: getNextId(),
        data: null,
      })
      return newSteps
    })
    onSubmit(formData)
  }

  const goBack = () => {
    setPageIndex((i) => i - 1)
  }

  const deletePage = (pageIndexToDelete) => {
    setSteps((steps) => steps.filter((_, i) => i !== pageIndexToDelete))
  }

  const goToFirstPageOfType = (pageType) => {
    let matchingPageIndex = steps.findIndex((step) => step.type === pageType)
    let goToPage = Math.max(matchingPageIndex, pageIndex - 1, 0)
    setPageIndex(goToPage)
  }

  return (
    <Component
      key={currentPage.id}
      currentStepData={currentPage.data}
      currentStepIndex={pageIndex}
      steps={steps.map((step, index) => ({ ...step, index }))}
      submit={onSubmit}
      repeatPage={onRepeatPage}
      savePageState={setPageData}
      goBack={goBack}
      goToPage={(index) => setPageIndex(index)}
      goToFirstPageOfType={goToFirstPageOfType}
      deletePage={deletePage}
      flowName={flowName}
      {...extraProps}
    />
  )
}
