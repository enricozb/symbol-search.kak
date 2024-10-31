/* @pareto-engineering/generator-front 1.0.12 */
import * as React from 'react'

import { useState, useMemo } from 'react'

import { Formik, Form } from 'formik'

import { useDispatch } from 'react-redux'

import { generatePath, useNavigate } from 'react-router-dom'

import { format } from 'date-fns'

import PropTypes from 'prop-types'

import styleNames from '@pareto-engineering/bem/exports'

// Local Definitions

import {
  Button,
  ExpandableLexicalPreview,
  People,
  TextareaInput,
} from '@pareto-engineering/design-system'

import { nodeTransformers } from 'modules/dashboard/common/transformers'

import {
  CyclicCarousel,
  CollapsableSidePanel,
  ExpandableContent,
  TimeElapsed,
  StickyFooter,
  QuerySelect,
  pushNotification,
  pushNotifications,
  Breadcrumbs,
  LabelingInterface,
} from 'modules/components'

import { useFormatComsLink } from 'utils'

import {
  getEndpoint, useQuery, useMutation, useCachedData, useRefreshCache,
  getText,
} from 'services'

import { Batch } from 'modules/dashboard/dataModel/models'

import './styles.scss'

const baseClassName = styleNames.base

const componentClassName = 'submission-review'

const globalText = getText('globals')

const warningText = getText('warnings')

const statusText = getText('submissionStatus')

const componentText = getText('submissionReview')

const submissionStatusesEndpoint = getEndpoint({
  resource   :'submission',
  subResource:'statuses',
})

function hello() {
}

class MyClass {
}

const SubmissionReview = ({
  id,
  className:userClassName,
  style,
  projectId,
  batchId,
  submissionId,
  singleViewEndpoint,
  listViewEndpoint,
  attachmentsEndpoint,
  listViewCacheKey,
  validStatuses:validStatusesProp,
  pageText,
  path,
  readOnlyTrigger,
}) => {
  const { data } = useQuery({
    endpointPath:singleViewEndpoint,
  })

  const navigate = useNavigate()

  const {
    title,
    timeElapsed,
    description,
    labelingConfig,
    instructions,
    internalComsChannelLink,
    projectOwner,
    worker,
    createdAt,
    data:submissionData,
    interfaceType,
    taskData,
  } = data

  const cachedSubmissions = useCachedData(listViewCacheKey)

  const { data:cachedSubmissionsData } = useQuery({
    endpointPath:!cachedSubmissions?.length ? listViewEndpoint : null,
  })

  const submissionsToCycle = cachedSubmissions.length
    ? cachedSubmissions : cachedSubmissionsData?.results ?? []

  const currentSubmissionIndex = useMemo(
    () => submissionsToCycle.findIndex(
      (submission) => submission.id === Number(submissionId)),
    [submissionsToCycle, submissionId])

  const nextSubmission = useMemo(
    () => submissionsToCycle[currentSubmissionIndex + 1],
    [submissionsToCycle, currentSubmissionIndex],
  )

  const previousSubmission = useMemo(
    () => submissionsToCycle[currentSubmissionIndex - 1],
    [submissionsToCycle, currentSubmissionIndex],
  )

  const goToNextSubmission = () => {
    const newPath = generatePath(path, {
      batchId,
      projectId,
      submissionId:nextSubmission?.id,
    })
    navigate(newPath)
  }

  const goToPreviousSubmission = () => {
    const newPath = generatePath(path, {
      batchId,
      projectId,
      submissionId:previousSubmission?.id,
    })
    navigate(newPath)
  }

  const navigateThroughSubmissions = () => {
    if (nextSubmission?.id) {
      goToNextSubmission()
    }
  }

  const [hasWidthCompressedEnded, setHasWidthCompressedEnded] = useState(false)

  const dispatch = useDispatch()

  const { icon, linkText } = useMemo(() => (
    useFormatComsLink(internalComsChannelLink)
  ), [internalComsChannelLink])

  const refreshCache = useRefreshCache()

  const { trigger, isMutating } = useMutation({
    endpointPath:singleViewEndpoint,
    onSuccess   :() => {
      dispatch(pushNotification({
        message:pageText.feedbackSuccessMessage,
        type   :'success',
      }))
      refreshCache([listViewEndpoint])

      navigateThroughSubmissions()
    },
    onError:(error) => {
      dispatch(pushNotifications(error.info))
    },
  })

  const { data: attachments } = useQuery({ endpointPath: attachmentsEndpoint })

  const taskAttachments = attachments?.filter((attachment) => attachment.source.toLowerCase() === 'task')

  const submissionAttachments = attachments?.filter((attachment) => attachment.source.toLowerCase() === 'submission')

  const isReadOnly = readOnlyTrigger && readOnlyTrigger(data)

  const hasParts = data.parts?.length > 0
  if (hasParts) {
    data.parts.sort((a, b) => a.index - b.index)
  }

  const formNames = hasParts ? data.parts.map((_, i) => [`partsReviewStatus[${i}]`, `partsFeedback[${i}]`]) : [['status', 'feedback']]
  const validStatuses = hasParts ? [
    statusText.APPROVED,
    statusText.REJECTED,
    statusText.PENDING_REVIEW,
  ] : validStatusesProp
  const isInterfaceXml = interfaceType === Batch.interfaceTypes.XML

  return (
    <div
      id={id}
      className={[
        baseClassName,
        componentClassName,
        userClassName,
      ]
        .filter((e) => e)
        .join(' ')}
      style={{
        '--panel-max-width'    :'25rem',
        '--controls-top-offset':'6rem',
        ...style,
      }}
      key={submissionId}
    >
      <Breadcrumbs
        path={path}
        values={{
          projectId,
          batchId,
          submissionId,
          title,
        }}
      />
      <div className="review-container">
        <div
          onAnimationEnd={(event) => {
            if (event.animationName === 'compressWidth') {
              setHasWidthCompressedEnded(true)
            } else if (event.animationName === 'expandWidth') {
              setHasWidthCompressedEnded(false)
            }
          }}
          className={`preview-container ${hasWidthCompressedEnded ? 'expand-width' : ''}`}
        >
          <div className="labeling-config-panel">

            {isInterfaceXml && (
            <ExpandableLexicalPreview
              header={(
                <div className="title">
                  <p className="h2">
                    {pageText.lexicalTitle}
                  </p>
                  <div className="review-metadata">
                    <div className="reviewee">
                      <span className="c-x x-highlighted">
                        {pageText.reviewingText}
                      </span>
                      <People columnWidth="8em" imageSize="1.5em">
                        <People.Person
                          key={worker?.firstName ?? worker?.id ?? ''}
                          name={`${worker?.firstName ?? ''} ${worker?.lastName ?? ''}`}
                          image={worker?.profilePicture}
                        />
                      </People>
                    </div>
                    <TimeElapsed
                      time={nodeTransformers.paddedSecondsToTime(timeElapsed)}
                    />
                  </div>
                </div>
              )}
              pageTitle={pageText.lexicalPageTitle}
              nodes={instructions}
              name="instructions"
              onBlock={() => dispatch(pushNotification({
                message:warningText.TAB_CANNOT_BE_OPENED,
                type   :'warning',
              }))}
            />
            )}
            <LabelingInterface
              interfaceId="label-studio-submission-review"
              interfaceType={interfaceType}
              labelStudioProps={{
                task:{
                  annotations:submissionData,
                  data       :taskData,
                },
                labelingConfig,
              }}
              formBuilderProps={{
                submissionData,
                attachments:submissionAttachments,
              }}
              taskAttachments={taskAttachments}
              readOnly
            />

          </div>
          <Formik
            initialValues={{
              feedback         :data.feedback,
              status           :data.status,
              partsFeedback    :data.parts.map(({ feedback }) => feedback),
              partsReviewStatus:data.parts.map(({ reviewStatus }) => reviewStatus),
            }}
            onSubmit={({
              feedback, status, partsFeedback, partsReviewStatus,
            }) => {
              const dataToSubmit = { timeElapsed }

              if (hasParts) {
                dataToSubmit.parts = partsReviewStatus.map((reviewStatus, index) => ({
                  submission:submissionId,
                  index,
                  reviewStatus,
                  feedback  :partsFeedback[index],
                }))
              } else {
                dataToSubmit.status = status
                dataToSubmit.feedback = feedback
              }

              trigger({ method: 'PUT', data: dataToSubmit })
            }}
          >
            {({ values }) => (
              <>
                <div className="form-container">
                  <Form id="submit-feedback">
                    {formNames.map(([status, feedback], i) => (
                      <>
                        {hasParts && (
                        <p className="h3">
                          {componentText.PART_LABEL}
                          {' '}
                          {i + 1}
                        </p>
                        )}

                        <QuerySelect
                          name={status}
                          label={componentText.STATUS_LABEL}
                          endpointPath={submissionStatusesEndpoint}
                          transformResponse={(payload) => (
                            payload?.results?.filter((statusObject) => (
                              validStatuses ? validStatuses.includes(statusObject.label) : true),
                            ).sort(
                              (a, b) => (
                                validStatuses.indexOf(a.label) - validStatuses.indexOf(b.label)
                              ),
                            ))}
                          optionsKeyMap={{
                            value   :'label',
                            getLabel:(statusObj) => statusObj.label,
                          }}
                          disabled={isReadOnly}
                        />

                        <TextareaInput
                          name={feedback}
                          label="Feedback"
                          placeholder={pageText.feedbackPlaceholder}
                          disabled={isReadOnly}
                        />
                      </>
                    ))}
                  </Form>
                </div>

                <CyclicCarousel
                  goToNext={goToNextSubmission}
                  goToPrevious={goToPreviousSubmission}
                  disableNext={!nextSubmission?.id}
                  disablePrevious={!previousSubmission?.id}
                />

                <StickyFooter
                  style={{
                    '--justify-content':'flex-end',
                  }}
                >
                  <Button
                    isGradient
                    color="interactive"
                    type="submit"
                    form="submit-feedback"
                    isLoading={isMutating}
                    disabled={isMutating || isReadOnly || (hasParts
                      ? !values.partsReviewStatus.every((status, index) => (
                        (status === statusText.APPROVED)
                        || (status === statusText.REJECTED && values.partsFeedback[index])
                      ))
                      : (!values.status || !values.feedback))}
                  >
                    {globalText.SUBMIT}
                  </Button>
                </StickyFooter>
              </>
            )}
          </Formik>

        </div>

        <CollapsableSidePanel
          className="span-4 s-1"
          style={{
            '--expanded-offset':'1rem',
          }}
        >
          <div className="grid pairs panel-item">
            <span className="panel-item-label">
              {globalText.SUBMITTED}
            </span>
            <span className="panel-item-value x-paragraph c-x">{format(new Date(createdAt), 'HH:mm:ss a - MM/dd/yyyy')}</span>
          </div>

          <div className="grid pairs panel-item">
            <span className="panel-item-label">
              {globalText.SUBMITTED_BY}
            </span>
            <span className="panel-item-value">
              <People columnWidth="8em" imageSize="2em">
                <People.Person
                  key={worker?.firstName ?? worker?.id ?? ''}
                  name={`${worker?.firstName ?? ''} ${worker?.lastName ?? ''}`}
                  image={worker?.profilePicture}
                />
              </People>
            </span>
          </div>

          <hr />

          <div className="grid pairs panel-item">
            <span className="panel-item-label">
              {pageText.projectOwnerLabel}
            </span>
            <span className="panel-item-value">
              <People columnWidth="8em" imageSize="2em">
                <People.Person
                  key={projectOwner?.firstName ?? projectOwner?.id ?? ''}
                  name={`${projectOwner?.firstName ?? ''} ${projectOwner?.lastName ?? ''}`}
                  image={projectOwner?.profilePicture}
                />
              </People>
            </span>
          </div>

          <div className="grid pairs panel-item">
            <span className="panel-item-label">
              {pageText.internalComsLabel}
            </span>
            <a
              href={internalComsChannelLink}
              className="panel-item-value x-paragraph c-x"
              target="_blank"
              rel="noreferrer"
            >
              <span className="icon s1">
                {icon}
              </span>
              <span>
                {linkText}
              </span>
            </a>
          </div>

          <hr />

          <div className="batch-description">
            <p>
              {pageText.descriptionLabel}
            </p>
            <ExpandableContent color="background-far">
              {description}
            </ExpandableContent>
          </div>

        </CollapsableSidePanel>
      </div>
    </div>
  )
}

SubmissionReview.propTypes = {
  /**
   * The HTML id for this element
   */
  id:PropTypes.string,

  /**
   * The HTML class names for this element
   */
  className:PropTypes.string,

  /**
   * The React-written, css properties for this element.
   */
  style:PropTypes.objectOf(PropTypes.string),
}

SubmissionReview.defaultProps = {
  // someProp:false
}

export default SubmissionReview
