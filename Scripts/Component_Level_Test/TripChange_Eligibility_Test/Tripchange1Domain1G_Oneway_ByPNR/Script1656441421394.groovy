import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint

import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testcase.TestCaseFactory.*
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

	
	//GlobalVariable.PNR = '1THLRG'

	
	
	
	//CustomKeywords.'methods.APIKeywords.TerminalPNR'()
	CustomKeywords.'methods.APIKeywords.TestExecution'() 
	
	
	def Eligibility = WS.sendRequestAndVerify(findTestObject('Object Repository/OneWayTrip/S_GetEligibilityRequest'))
		
	CustomKeywords.'methods.APIKeywords.ErrorMessage'(Eligibility)

	def slurper = new groovy.json.JsonSlurper()

	def sEligibility = slurper.parseText(Eligibility.getResponseBodyContent())

	def sResponse = sEligibility.findAll()

	sResponse.each {

		def sKeys =  " $it.key "

		sKeys = sKeys.toString().trim()


		KeywordUtil.markPassed('-------Validation for EligibilitybyPNR-----------')

		if (sKeys == 'TicketChangeEligibilityListResponse') {


			def sExchangable = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].exchangeable

			if (sExchangable == 'All' || sExchangable == 'Some')

			{

				KeywordUtil.markPassed('Exchangeable Value is available in Response as :- '+ sExchangable)



				String sPTC = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].PassengerTypeCode

				if (sPTC.length() > 0 && sPTC != null) {

					KeywordUtil.markPassed('PTC Value is available in Response as:- '+ sPTC)

				} else {

					KeywordUtil.markFailed('Error Response returned or PTC Value is is not available')
				}


				String sTicketValue = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].Identifier.value

				if (sTicketValue.length() > 0 && sTicketValue != null) {

					KeywordUtil.markPassed('TicketNumber Value is available in Response as :-' + sTicketValue)


				} else {
					KeywordUtil.markFailed('Error Response returned or ticketNumber Value is is not available')

				}


				String sTraceID =sEligibility.TicketChangeEligibilityListResponse.traceId


				if (sTraceID.length() > 0 && sTraceID != null) {

					KeywordUtil.markPassed('traceid Value is available in Response as :-' + sTraceID)


				} else {
					KeywordUtil.markFailed('Error Response returned or traceid Value is is not available')

				}




				String sMaxChangeFee = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].Penalties.Change[0].Penalty[0].Amount.value

				if (sMaxChangeFee.length() > 0 && sMaxChangeFee != null) {

					KeywordUtil.markPassed('maximumChangeFee value  is available in Response as :- ' +sMaxChangeFee)


				} else {

					KeywordUtil.markFailed('Error Response returned or maximumChangeFee value  is is not available')

				}




				String sMinChangeFee = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibility[0].Penalties.Change[1].Penalty[0].Amount.value

				if (sMinChangeFee.length() > 0 && sMinChangeFee != null) {

					KeywordUtil.markPassed('minimumChangeFee value  is available in Response as :- ' +sMinChangeFee)


				} else {

					KeywordUtil.markFailed('Error Response returned or minimumChangeFee value  is is not available')

				}



			}

			else

			{


				KeywordUtil.markFailedAndStop('Error Response returned or Exchangeable Value is is not available')
				//KeywordUtil.logInfo('The PNR is not exchangeable')

			}
		 
			
			def sRefundable = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].refundable
			
			if ((sRefundable.length() > 0) && (sRefundable != null)) {
				KeywordUtil.markPassed('Refundable value  is available in Response as :- ' + sRefundable) //KeywordUtil.logInfo('The PNR is not exchangeable')
			} else {
				KeywordUtil.markFailed('Error Response returned or Refundable value  is is not available')
			}


		}
	}

